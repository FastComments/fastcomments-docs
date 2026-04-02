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

Then add the dependencies you need:

```xml
<dependencies>
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Bibliothèque Core (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // Client API
    implementation "com.fastcomments:client:1.3.2"
    
    // Bibliothèque Core (inclut SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java Core qui contient des utilitaires écrits à la main
pour faciliter le travail avec l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de changements.

- [Docs de la bibliothèque client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque Core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Docs de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs publiques vs sécurisées

Pour le client API, il y a deux classes, `DefaultApi` et `PublicApi`. `DefaultApi` contient des méthodes qui requièrent votre clé API, et `PublicApi` contient des
appels d'API qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc. sans authentification.