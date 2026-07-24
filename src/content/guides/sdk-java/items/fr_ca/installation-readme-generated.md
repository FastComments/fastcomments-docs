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

Ensuite, ajoutez les dépendances dont vous avez besoin :

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

Ajoutez le référentiel Repsy à votre fichier **build.gradle** :

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

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java principale qui contient des utilitaires écrits à la main pour faciliter l’utilisation de l’API, et le module `pubsub` qui est une bibliothèque pour s’abonner aux flux de changements.

- [Documentation de la bibliothèque client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque principale, y compris les exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiques vs sécurisées

Le client API comprend trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification.

Le `ModerationApi` offre une suite étendue d’API de modération en direct et rapides. Chaque méthode du `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.