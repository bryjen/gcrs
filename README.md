<div align="center">
    <div align="center">
        <h1 align="center"><b>gcrs</b></h1>
            <div align="center" style="display: flex; justify-content: center; gap: 1rem; margin-bottom: 1rem;">
            <img width="400" alt="Image" src="https://github.com/user-attachments/assets/a7b458e1-0ee7-4859-a41f-469893aae8b6" />
            <img width="400" alt="Image" src="https://github.com/user-attachments/assets/fe492b93-8a19-45e1-8e5a-e4c4da267523" />
            </div>
        <br/>
        POC Git hosting web-app service built using Rust + Leptos (server & WASM).
    </div>
</div>


> [!CAUTION]
> This project is not fully in active development, I may come back to this project once and a while. For more information, see below.

## Thoughts

The premise of Leptos was that you can have SSSR-ed paged, whilst also seamlessly integrating client-side reactivity using its 'Islands' feature. Additionally, these client-side islands (running on WASM) would be able to call essentially "server functions", which would create a stub on the client that would calls the server code via HTTP (im pretty sure). All of this, allows you to effectively write interactive HTML whilst also having backend logic in Rust, using its strong static type system and memory guarantees.

That all sounds good on paper, and from the experience so far with this project, it **is** able to fullfill all of its aforementioned promises. However, the biggest problem is primarily the overall complexity and the lack of tooling for both Rust and Leptos. Although this most certainly is a skill issue, I don't think that manually managing memory on the frontend side is a particulalry fond experience; that, on top of deeply nested trait types (like `RwSignal<Vec<Arc<String>>>` to represent stateful owned strings that can be used by multiple components) and lifetimes.

Furthermore, one of the biggest problems is the abhorrent compile times. Despite taking multiple steps to optimize the compilation speed of the project, it still takes 3-5+ minutes for a full compilation, and 5-10s when watching the project, even after specifying `--hot-reload`.

A very solid library, but i'm very much skill issue-d as of right now.

## Running

To guarantee that the project builds without any other problems, ensure that you have [`nix`](https://nixos.org/download/) installed. From there, the following should just work:

```shell
nix develop
cargo leptos serve # or watch --hot-reload
```
