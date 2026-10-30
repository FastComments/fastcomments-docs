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
    <!-- Klijent API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Osnovna biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub biblioteka (za događaje u stvarnom vremenu) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    // Klijent API
    implementation "com.fastcomments:client:3.1.1"
    
    // Osnovna biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub biblioteka (za događaje u stvarnom vremenu)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Dokumentacija biblioteke klijenta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains methods that can be made directly from a browser/mobile device/etc without authentication.

The `ModerationApi` provides an extensive suite of live and fast moderation APIs. Every `ModerationApi` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.