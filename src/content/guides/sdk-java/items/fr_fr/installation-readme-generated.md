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

Ajoutez ensuite les dépendances dont vous avez besoin :

```xml
<dependencies>
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Bibliothèque Core (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
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
    // Client API
    implementation "com.fastcomments:client:3.1.2"
    
    // Bibliothèque Core (inclut SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Contenu de la bibliothèque

Cette bibliothèque contient trois modules : le client API généré, la bibliothèque Java core qui contient des utilitaires écrits à la main
pour faciliter le travail avec l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de changements.

- [Documentation de la bibliothèque client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiques vs API sécurisées

Pour le client API, il existe trois classes : `DefaultApi`, `PublicApi` et `ModerationApi`. La classe `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes
qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification.

La classe `ModerationApi` fournit une suite étendue d'API de modération en direct et rapides. Chaque méthode de `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.