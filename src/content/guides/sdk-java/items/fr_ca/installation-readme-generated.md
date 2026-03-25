### Maven

Ajoutez le dépôt Repsy au POM de votre projet :

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Ensuite, ajoutez les dépendances dont vous avez besoin :

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

Ajoutez le dépôt Repsy à votre fichier build.gradle :

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

### Library Contents

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java core qui contient des utilitaires écrits à la main
pour faciliter le travail avec l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de changements.

- [Documentation de la bibliothèque du client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque Core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Pour le client API, il y a deux classes, `DefaultApi` et `PublicApi`. La `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels d'API
qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc. sans authentification.