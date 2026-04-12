use tokio::net::UdpSocket;
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    // Kokeile porttia 6791, joka on MeshCom-standardi
    let target_addr = "192.168.68.144:1799"; 

    // let payload = json!({
    //     "type": "msg",
    //     "src": "OH6CXK-10",
    //     "dst": "ALL",
    //     "msg": "K B4 A"
    // });

    // let payload = json!({
    //     "type": "msg",
    //     "src_type": "node",  // Lisätty: kertoo että lähettäjä on solmu
    //     "src": "OH6CXK-10",
    //     "dst": "ALL",
    //     "msg": "K B4 A"
    // });

    // let payload = json!({
    //     "src_type": "lora",  // Kokeile huijata: väitä että viesti tuli radiolta
    //     "type": "msg",
    //     "src": "OH6CXK-10",
    //     "dst": "ALL",
    //     "msg": "K B4 A",
    //     "msg_id": "B001E0FF" // Lisätty kuvitteellinen ID
    // });

    let payload = json!({
        // "src_type": "node", // Kerrotaan, että viesti tulee solmulta
        "src_type": "lora", // Väitä, että viesti tuli radiolta
        "type": "msg",      // Viestityyppi: normaali viesti
        "src": "OH6CXK-10", // Lähettäjä

        // "dst": "*",      // Kohde: Kaikki (Broadcast)
        "dst": "OH6CXK-2",  // Kohde vain OH6CXK-2

        "msg": "K B4 A",    // Itse komento
        "msg_id": "A1B2",   // Lisätään ID, jotkut versiot vaativat tämän
        "via": "EXT"        // Kerrotaan, että reitti on External
    });


    let data = payload.to_string();
    socket.send_to(data.as_bytes(), target_addr).await?;

    println!("Lähetetty kohteeseen {}: {}", target_addr, data);
    Ok(())
}


