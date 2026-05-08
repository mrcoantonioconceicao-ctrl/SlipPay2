#!/data/data/com.termux/files/usr/bin/bash

cd ~/slippay-rust || exit

echo "🔍 Verificando alterações..."

if [[ -z $(git status --porcelain) ]]; then
    echo "✅ Nenhuma alteração encontrada."
    exit 0
fi

git add .

DATE=$(date "+%Y-%m-%d %H:%M:%S")

git commit -m "auto update $DATE"

echo "📥 Sincronizando com GitHub..."

git pull origin main --rebase

echo "🚀 Enviando alterações..."

if git push origin main; then
    echo "✅ SlipPay sincronizado com GitHub!"
else
    echo "❌ Falha no push!"
fi
