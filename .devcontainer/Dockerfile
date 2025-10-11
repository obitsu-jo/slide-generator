# Rustの公式イメージをベースに使用
FROM rust:latest

# システムパッケージの更新と、開発ツールおよびライブラリのインストール
RUN apt-get update && apt-get install -y \
    # 開発ツール
    git tree \
    # aptキャッシュのクリーンアップ
    && rm -rf /var/lib/apt/lists/*

# devcontainerが起動し続けるようにするためのコマンド
CMD ["sleep", "infinity"]