### Maven

Ajoutez le référentiel Repsy au POM de votre projet :

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
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Bibliothèque Core (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le référentiel Repsy à votre fichier **build.gradle** :

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Client API
    implementation "com.fastcomments:client:3.0.0"
    
    // Bibliothèque Core (inclut SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules : le client API généré, la bibliothèque Java Core qui comprend des utilitaires écrits à la main pour faciliter l’utilisation de l’API, et le module `pubsub` qui est une bibliothèque pour s’abonner aux flux de changements.

- [Documentation de la bibliothèque client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque Core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. La classe `DefaultApi` contient des méthodes qui requièrent votre clé API, tandis que `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur, un appareil mobile, etc., sans authentification.

La classe `ModerationApi` offre une suite étendue d’APIs de modération en temps réel et à grande vitesse. Chaque méthode `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.