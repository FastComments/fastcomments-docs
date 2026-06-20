### Maven

プロジェクトの POM に Repsy リポジトリを追加します:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

次に必要な依存関係を追加します:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradle ファイルに Repsy リポジトリを追加します:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

このライブラリは3つのモジュールを含みます。生成された API クライアント、API の操作を容易にする手書きのユーティリティを含むコア Java ライブラリ、そして変更フィードの購読用ライブラリである `pubsub` モジュールです。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API クライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の3つのクラスがあります。`DefaultApi` は API キーを必要とするメソッドを含み、`PublicApi` はブラウザ／モバイル端末等から認証なしで直接実行できるメソッドを含みます。

`ModerationApi` はモデレーター用ダッシュボードを支えます。コメントのモデレーションに関するメソッド（list、count、search、logs、export）、モデレーション操作（remove/restore、
flag、review/spam/approval ステータスの設定、votes、スレッドの再開/クローズ）、バン関連（コメントからのバン、バンの解除、事前バン概要、バンのステータスと設定、バンされたユーザー数）、
およびバッジと信頼（バッジの付与/削除、手動バッジ、信頼係数の取得/設定、ユーザー内部プロファイル）を含みます。すべての `ModerationApi` メソッドは `sso` パラメータを受け取るため、呼び出しを SSO で認証されたモデレーターを代理して実行できます。