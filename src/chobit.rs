use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use reqwest::{IntoUrl, Url};
use scraper::{node::Element, ElementRef, Html, Selector};

use crate::scraper_ext::TryAttr;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VoiceSampleTrack {
    pub media_url: Url,
    pub title: String,
    pub playtime: String, // Todo to Duration
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VoiceProduct {
    pub product_id: String,
    pub title: String,
    pub circle_name: String,
    pub cover_url: Url,
    pub voice_sample_tracks: Vec<VoiceSampleTrack>,
}

impl VoiceSampleTrack {
    fn from_track_li_elm(elm: &Element) -> Result<VoiceSampleTrack> {
        let title = elm.try_attr("data-title")?;
        let src = elm.try_attr("data-src")?;
        let playtime = elm.try_attr("data-playtime")?;

        let media_url = Url::parse(src)?;
        Ok(VoiceSampleTrack {
            media_url,
            title: title.into(),
            playtime: playtime.into(),
        })
    }
}

impl VoiceProduct {
    fn from_playerbox_div(div: ElementRef) -> Result<VoiceProduct> {
        static TRACK_LIST_SELECTOR: Lazy<Selector> =
            Lazy::new(|| Selector::parse("div.track-list > ol > li").unwrap());
        let maker = div.value().try_attr("data-album-maker")?;
        let title = div.value().try_attr("data-album-title")?;
        let id = div.value().try_attr("data-file-relation-id")?;
        let cover_url = div.value().try_attr("data-albumart")?;
        let cover_url = Url::parse(cover_url)?;

        let voice_sample_tracks = div
            .select(&TRACK_LIST_SELECTOR)
            .into_iter()
            .flat_map(|elm| VoiceSampleTrack::from_track_li_elm(elm.value()))
            .collect::<Vec<_>>();

        Ok(VoiceProduct {
            product_id: id.into(),
            title: title.into(),
            circle_name: maker.into(),
            cover_url,
            voice_sample_tracks,
        })
    }

    fn from_document(doc: Html) -> Result<VoiceProduct> {
        static PLAYER_BOX_SELECTOR: Lazy<Selector> =
            Lazy::new(|| Selector::parse("div.player-box").unwrap());

        let mut playerbox = doc.select(&PLAYER_BOX_SELECTOR);
        let Some(elm) = playerbox.nth(0) else {
            bail!("player-box not found")
        };
        VoiceProduct::from_playerbox_div(elm)
    }

    async fn from_url<T: IntoUrl>(url: T) -> Result<VoiceProduct> {
        let text = reqwest::get(url).await?.text().await?;
        let doc = Html::parse_document(&text);
        VoiceProduct::from_document(doc)
    }
}

#[tokio::test]
async fn test_from_document() {
    let product = VoiceProduct::from_url("https://chobit.cc/g1y57")
        .await
        .unwrap();
    assert_eq!(
        product.title,
        "【ブルーアーカイブ】ノノミASMR～ほのかな体温を感じる距離～\u{3000}体験版".to_string()
    );
    assert_eq!(product.voice_sample_tracks, vec![
        VoiceSampleTrack { title: "01.『タイトルコール』".into(), media_url: "https://file.chobit.cc/contents/2201/9msjbsuadj408040g80k40wsg/9msjbsuadj408040g80k40wsg_001.m4a".parse().unwrap(), playtime: "00:17".into() },
        VoiceSampleTrack { title: "02.『ちょっとした悪戯です』".into(), media_url: "https://file.chobit.cc/contents/2201/9msjbsuadj408040g80k40wsg/9msjbsuadj408040g80k40wsg_002.m4a".parse().unwrap(), playtime: "09:27".into() },
    ]);
}
