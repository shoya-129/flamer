```flame
import flamer



fn login(body: Formula) -> Formula {
    return body
}

@Flamer(port: 3000)
async fn main() {
    flamer.get(
        "/", 
        () {
            "hello"
        }
    )
    flamer.post("/login", login)
    
    await flamer.listen()
}

await main()

```
