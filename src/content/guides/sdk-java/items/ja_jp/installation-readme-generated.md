### Maven

プロジェクトのPOMにRepsyリポジトリを追加してください:

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
    <!-- API クライアント -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- コアライブラリ (SSO を含む) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub ライブラリ (ライブイベント用) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // API クライアント
    implementation "com.fastcomments:client:1.3.2"
    
    // コアライブラリ (SSO を含む)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub ライブラリ (ライブイベント用)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### ライブラリの内容

このライブラリは3つのモジュールで構成されています。生成された API クライアント、API を使いやすくするための手書きユーティリティを含むコア Java ライブラリ、そして変更フィードの購読用ライブラリである `pubsub` モジュールです。

- [API クライアント ライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [コアライブラリのドキュメント（SSO の例を含む）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub ライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API と保護された API

API クライアントには `DefaultApi` と `PublicApi` の2つのクラスがあります。`DefaultApi` は API key が必要なメソッドを含み、`PublicApi` はブラウザやモバイルデバイス等から認証なしで直接実行できる API 呼び出しを含みます。