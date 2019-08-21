#![feature(async_await)]

use igdb_client::client::IGDBClient;
use igdb_client::model::games::Game;
use igdb_client::request_builder::Equality;

fn main() {
    async_std::task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");

        let games_client = igdb_client.games();

        let mut witcher_request = IGDBClient::create_request();
        witcher_request
            .add_fields(vec![
                "name",
                "summary",
                "category",
                "status",
                "url",
                "total_rating",
                "involved_companies",
                "websites",
            ])
            .search("Witcher")
            .limit(3);

        let result = games_client.get(&witcher_request).await.unwrap();
        //show_games(&result);

        let witcher = result.first().unwrap();

        let company_client = igdb_client.companies();
        let company_id = witcher.involved_companies.first().unwrap().to_string();

        println!("Company: {:?}", witcher);
        println!("Company: {}", company_id);

        let mut company_request = IGDBClient::create_request();
        company_request
            .all_fields()
            .add_where("id", Equality::Equal, company_id);

        let companies = company_client.get(&company_request).await.unwrap();
        let company = companies.first().unwrap();
        println!("{:?}", company);

        let website_client = igdb_client.websites();
        let mut website_req = IGDBClient::create_request();

        website_req
            .all_fields()
            .add_where("game", Equality::Equal, witcher.id.to_string());

        let websites = website_client.get(&website_req).await.unwrap();

        println!("Website for witcher is: {:?}", websites.first().unwrap());
    })
}

#[allow(dead_code)]
fn show_games(games: &Vec<Game>) {
    for game in games {
        println!("Name: {}", game.name);
        println!("Summary: {}", game.summary);
        println!("Url: {}", game.url);
        println!("Total rating: {}", game.total_rating);
    }
}
