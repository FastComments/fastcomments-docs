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
    <!-- Cliente de la API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Biblioteca Core (incluye SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos en vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
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
    // Cliente de la API
    implementation "com.fastcomments:client:2.0.0"
    
    // Biblioteca Core (incluye SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // Biblioteca PubSub (para eventos en vivo)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Documentación de la biblioteca del cliente de la API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la biblioteca Core, incluyendo ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains methods
that can be made directly from a browser/mobile device/etc without authentication.

The `ModerationApi` powers the moderator dashboard. It contains methods for comment moderation (list, count, search, logs, and export), moderation actions (remove/restore,
flag, set review/spam/approval status, votes, and reopen/close thread), bans (ban from comment, undo a ban, pre-ban summaries, ban status and preferences, and banned-user counts),
and badges & trust (award/remove a badge, manual badges, get/set trust factor, and user internal profile). Every `ModerationApi` method accepts an `sso` parameter so the call can be
performed on behalf of an SSO-authenticated moderator.