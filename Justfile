web: typeshare
    npm run dev

typeshare:
    typeshare-cli fish-engine fish-wasm -c=typeshare.toml --lang=typescript --output-file=./src/types.ts

bench:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root
