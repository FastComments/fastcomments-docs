### Maven

プロジェクトのPOMにRepsyリポジトリを追加します:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

次に、必要な依存関係を追加します:

```xml
<dependencies>
    <!-- APIクライアント -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- コアライブラリ(SSOを含む) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSubライブラリ(ライブイベント用) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradleファイルにRepsyリポジトリを追加します:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // APIクライアント
    implementation "com.fastcomments:client:3.1.2"
    
    // コアライブラリ(SSOを含む)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSubライブラリ(ライブイベント用)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### ライブラリの内容

このライブラリには3つのモジュールが含まれています。生成されたAPIクライアント、APIの操作を容易にするための手書きユーティリティを含むコアJavaライブラリ、そして変更フィードを購読するためのライブラリである`pubsub`モジュールです。

- [APIクライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [SSOの例を含むコアライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSubライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開APIとセキュアAPI

APIクライアントには、`DefaultApi`、`PublicApi`、`ModerationApi`の3つのクラスがあります。`DefaultApi`にはAPIキーが必要なメソッドが含まれており、`PublicApi`には認証なしでブラウザやモバイルデバイスなどから直接実行できるメソッドが含まれています。

`ModerationApi`は、ライブかつ高速なモデレーションAPIの充実したスイートを提供します。すべての`ModerationApi`メソッドは`sso`パラメータを受け入れ、SSOまたはFastComments.comのセッションクッキー経由で認証できます。