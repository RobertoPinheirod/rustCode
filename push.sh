#!/bin/bash
# Script para agilizar o salvamento no GitHub

# 1. Adiciona todas as mudanças
git add .

# 2. Pergunta pela mensagem do commit (para não ficar tudo igual)
echo "O que você mudou hoje, Roberto?"
read mensagem

# 3. Faz o commit com a sua mensagem
git commit -m "$mensagem"

# 4. Envia para o GitHub
git push

echo "Feito! Seu progresso em Rust está salvo no GitHub."
