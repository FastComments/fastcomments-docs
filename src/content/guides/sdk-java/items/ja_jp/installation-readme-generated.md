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
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Repsy リポジトリを `build.gradle` ファイルに追加します:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API クライアント
    implementation "com.fastcomments:client:3.0.0"
    
    // コア ライブラリ（SSO を含む）
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub ライブラリ（ライブ イベント用）
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

このライブラリには 3 つのモジュールが含まれています。生成された API クライアント、API の利用を容易にする手書きユーティリティを含むコア Java ライブラリ、そして変更フィードを購読するための `pubsub` モジュールです。

- [API クライアント ライブラリ ドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [コア ライブラリ ドキュメント（SSO の例を含む）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub ライブラリ ドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API クライアントには、`DefaultApi`、`PublicApi`、および `ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザやモバイル端末などから直接呼び出せるメソッドが含まれます。

`ModerationApi` は、ライブかつ高速なモデレーション API の幅広いスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーを使用して認証できます。