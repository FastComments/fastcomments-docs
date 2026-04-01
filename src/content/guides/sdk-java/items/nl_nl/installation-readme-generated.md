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

Voeg vervolgens de benodigde afhankelijkheden toe:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentatie van de core-bibliotheek, inclusief SSO-voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentatie van de PubSub-bibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Openbare versus beveiligde API's

For the API client, there are two classes, `DefaultApi` and `PublicApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.