# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL'

# Database
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'


# Watch
alias watch-app='mold -run cargo watch --workdir /workspace/ -w crates/web-pages -w crates/web-server -w crates/db -w crates/web-assets/dist -w crates/web-assets/images --no-gitignore -x "run --bin web-server"'
alias wa=watch-app
alias watch-pipeline='npm install --prefix /workspace/crates/web-assets && npm run start --prefix /workspace/crates/web-assets'
alias wp=watch-pipeline
alias watch-embeddings='mold -run cargo watch --workdir /workspace/ -w crates/embeddings-api -w crates/rag-engine --no-gitignore -x "run --bin rag-engine"'
alias we=watch-embeddings
alias watch-tailwind='cd /workspace/crates/web-assets && npx tailwindcss -i ./input.css -o ./dist/output.css --watch'
alias wt=watch-tailwind
alias watch-static='cd /workspace/crates/static-website && cargo watch --workdir /workspace/crates/static-website -w ./content -w ./src --no-gitignore -x "run --bin static-website"'
alias ws=watch-static
alias watch-tailwind-static='cd /workspace/crates/static-website && npx tailwindcss -i ./input.css -o ./dist/tailwind.css --watch'
alias wts=watch-tailwind-static

# Spell check
alias spell='docker run --rm -ti -v /workspace/crates/static-website/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"'

# k3d
alias k3d-create='k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"'
alias k3d-del='k3d cluster delete'
alias k3d-dev-setup='cargo run --bin supakube -- install --development --namespace bionic-gpt --operator-namespace bionic-system --app-name bionic-gpt --insecure-override-passwords testpassword --db-user-prefix bionic_'
alias k3d-postgres='kubectl apply -f /workspace/.devcontainer/kubernetes/postgres.yaml'
alias k3d-rag-engine='kubectl apply -f /workspace/.devcontainer/kubernetes/rag-engine.yaml'
alias k3d-smtp='kubectl apply -f /workspace/.devcontainer/kubernetes/smtp.yaml'
