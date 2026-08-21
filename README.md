# Flamer

***Express.js inspired backend framework for Flame.***

- *Leverages Rust **tokio** and **axum***
- *Easy to use inside **Flame***

---

## Instaltion

- Add flamer as a dependency by using the flame add command:

Terminal window

```shell
 flame add https://github.com/shoya-129/flamer
```

Alternatively, manually add it to your flame.toml:

```toml
[dependencies]
flamer = "https://github.com/shoya-129/flamer"
```

---

## Uses

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

---

## Telegram Bot example

```flame
import std.net.http
import std.json
import flamer

// Replace with your actual Bot Token from BotFather
let bot_token = "YOUR_BOT_TOKEN_HERE"

async fn webhook(body: Formula) -> Formula {
    // Parse the incoming JSON request body from Telegram
    let data = json.parse(body)

    // Extract the sender's Chat ID and the message text
    let chat_id = data.message.chat.id
    let text = data.message.text

    println($"Received from {chat_id}: {text}")

    let mut reply_text = ""

    // Simple command routing
    if text == "/start" {
        reply_text = "Welcome to Flame Bot! Send me a message."
    } else {
        reply_text = $"You said: {text}"
    }

    // Build the Telegram API request url
    let send_url = $"https://api.telegram.org/bot{bot_token}/sendMessage"

    // Prepare the JSON payload
    let payload = {
        chat_id: chat_id,
        text: reply_text
    }

    // Fire the HTTP POST request to Telegram!
    let send_res = await http.post(send_url, payload)

    // Print the API response for debugging
    println($"Telegram response: {send_res.text()}")

    // Return a successful HTTP 200 response
    return {
        ok: true
    }
}

@Flamer(port: 3000)
async fn main() {
    println("--- Flame Telegram Bot Webhook Server ---")
    println("Server listening on port 3000.")
    println("Point your Telegram webhook to: /webhook")

    // Setup routing
    flamer.post("/webhook", webhook)

    // Start listening asynchronously
    await flamer.listen()
}

await main()
```
