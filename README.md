# dx
https://github.com/manfromexistence/dx-forge.git
find . -maxdepth 1 -mindepth 1 -type d -exec du -sh {} + | sed 's/K/KB/; s/M/MB/; s|\./||'
find . -maxdepth 1 -mindepth 1 ! -name "cli" ! -name "src" ! -name "creates" ! -name "packages" -exec rm -rf {} +
npm install -g @anthropic-ai/claude-code


```
git clone https://github.com/anthropics/claude-code && cd claude-code && rm -rf .git && cd ..
git clone https://github.com/ratatui/ratatui && cd ratatui && rm -rf .git && cd ..
git clone https://github.com/google-gemini/gemini-cli && cd gemini-cli && rm -rf .git && cd ..
git clone https://github.com/mikaelmello/inquire && cd inquire && rm -rf .git && cd ..
git clone https://github.com/bombshell-dev/clack && cd clack && rm -rf .git && cd ..
git clone https://github.com/oven-sh/bun && cd bun && rm -rf .git && cd ..
git clone https://github.com/haydenbleasel/ultracite.git && cd ultracite && rm -rf .git && cd ..
git clone https://github.com/tailwindlabs/tailwindcss && cd tailwindcss && rm -rf .git && cd ..
```
