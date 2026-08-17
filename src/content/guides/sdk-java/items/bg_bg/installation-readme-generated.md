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
    <!-- API клиент -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Основна библиотека (включва SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub библиотека (за живи събития) -->
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
    // API клиент
    implementation "com.fastcomments:client:3.0.1"
    
    // Основна библиотека (включва SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub библиотека (за живи събития)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand‑written utilities to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Документация за API клиентската библиотека](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация за основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация за PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains methods that can be made directly from a browser/mobile device/etc without authentication.

The `ModerationApi` provides an extensive suite of live and fast moderation APIs. Every `ModerationApi` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.