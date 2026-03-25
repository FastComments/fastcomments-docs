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
    <!-- Клиент API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Библиотека Core (включает SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Библиотека PubSub (для событий в реальном времени) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    // Клиент API
    implementation "com.fastcomments:client:1.3.1"
    
    // Библиотека Core (включает SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Библиотека PubSub (для событий в реальном времени)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Содержимое библиотеки

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Документация библиотеки API-клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация Core-библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публичные и защищённые API

For the API client, there are two classes, `DefaultApi` and `PublicApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.