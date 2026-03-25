### Maven

Ajoutez le dépôt Repsy au POM de votre projet:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Puis ajoutez les dépendances dont vous avez besoin:

```xml
<dependencies>
    <!-- Client de l'API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Bibliothèque Core (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le dépôt Repsy à votre fichier build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Client de l'API
    implementation "com.fastcomments:client:1.3.1"
    
    // Bibliothèque Core (inclut SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Documentation de la bibliothèque du client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque Core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Pour le client API, il existe deux classes, `DefaultApi` et `PublicApi`. La `DefaultApi` contient des méthodes nécessitant votre clé API, et `PublicApi` contient des appels d'API qui peuvent être effectués directement depuis un navigateur/mobile/appareil/etc sans authentification.