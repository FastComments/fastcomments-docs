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

次に、必要な依存関係を追加します:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### ライブラリの内容

このライブラリは 3 つのモジュールで構成されています。生成された API クライアント、API の利用を容易にする手書きユーティリティを含むコア Java ライブラリ、そして変更フィードの購読用ライブラリである `pubsub` モジュールです。

- [API クライアント ライブラリ ドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [コア ライブラリ ドキュメント（SSO の例を含む）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub ライブラリ ドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API と保護された API

API クライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザやモバイルデバイスなどから直接呼び出せるメソッドが含まれます。

`ModerationApi` はライブかつ高速なモデレーション API の豊富なスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーを使用して認証できます。