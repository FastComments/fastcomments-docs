### Maven

Add the Repsy repository to your project's POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Add the Repsy repository to your build.gradle file:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

このライブラリは 3 つのモジュールで構成されています。生成された API クライアント、API の利用を容易にする手書きユーティリティを含むコア Java ライブラリ、そして変更フィードの購読用ライブラリである `pubsub` モジュールです。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API クライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザやモバイルデバイスなどから直接呼び出せるメソッドが含まれます。

`ModerationApi` はライブかつ高速なモデレーション API の豊富なスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーを使用して認証できます。