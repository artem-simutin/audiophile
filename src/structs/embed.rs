use serenity::builder::CreateEmbedFooter;

pub struct AudiophileEmbeds {}

impl AudiophileEmbeds {
    pub fn footer(f: &mut CreateEmbedFooter) -> &mut CreateEmbedFooter {
        f.icon_url("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Flogos-world.net%2Fwp-content%2Fuploads%2F2020%2F12%2FDiscord-Logo.png&f=1&nofb=1&ipt=a36703a34732bef88dea89daac58c42b1822ab502739d1759f3bbe1d173c2f71&ipo=images").text("Powered by Audiophile")
    }
}
