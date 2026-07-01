### Maven

Ajoutez le référentiel Repsy au POM de votre projet :

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Puis ajoutez les dépendances dont vous avez besoin :

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le référentiel Repsy à votre fichier `build.gradle` :

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules : le client API généré, la bibliothèque Java de base qui comprend des utilitaires écrits à la main pour simplifier l’utilisation de l’API, et le module `pubsub`, qui est une bibliothèque pour s’abonner aux flux de modifications.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur, un appareil mobile, etc. sans authentification.

`ModerationApi` fournit une suite étendue d’API de modération en temps réel et rapides. Chaque méthode `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.