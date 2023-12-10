use crate::soda::tmdb::{self, scraper};

use super::entity::{MTInfo, MTMetadata, ScrapeConfig};

/// 刮削要整理的资源
pub(crate) fn scrape_metadata(scrape_config: &ScrapeConfig, mt_meta: &MTMetadata, mt_info: &MTInfo, path: &str) {
    tracing::info!("scrape_metadata mt_info = {:?} path = {:?} ", mt_info.title(), path);
    scraper::gen_scrape_files(scrape_config, mt_meta, mt_info, path);
    tracing::info!("scrape_metadata success");
}
