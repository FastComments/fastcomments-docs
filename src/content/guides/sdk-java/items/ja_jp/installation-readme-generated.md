### Maven

プロジェクトの POM に Repsy リポジトリを追加してください:

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
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradle ファイルに Repsy リポジトリを追加してください:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### ライブラリの内容

このライブラリは3つのモジュールを含んでいます。生成された API クライアント、API の操作を容易にするための手書きユーティリティを含むコア Java ライブラリ、そして変更フィードの購読用ライブラリである `pubsub` モジュールです。

- [API クライアント ライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [コアライブラリのドキュメント（SSO の例を含む）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub ライブラリのドキュメント](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API と保護された API

API クライアントには `DefaultApi` と `PublicApi` の2つのクラスがあります。`DefaultApi` は API キーを必要とするメソッドを含み、`PublicApi` はブラウザやモバイルデバイス等から認証なしで直接実行できる API 呼び出しを含みます。