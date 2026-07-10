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
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Bibliothèque principale (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le dépôt Repsy à votre fichier **build.gradle** :

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Client API
    implementation "com.fastcomments:client:3.0.1"
    
    // Bibliothèque principale (inclut SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java principale qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de changements.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et le `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification.

Le `ModerationApi` fournit une suite étendue d'API de modération en temps réel et rapides. Chaque méthode du `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.