### Maven

Добавете репозитория Repsy към вашия POM файл на проекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете зависимостите, от които се нуждаете:

```xml
<dependencies>
    <!-- API клиент -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Основна библиотека (включва SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub библиотека (за живи събития) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Добавете репозитория Repsy към вашия файл `build.gradle`:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API клиент
    implementation "com.fastcomments:client:3.0.0"
    
    // Основна библиотека (включва SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub библиотека (за живи събития)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

Тази библиотека съдържа три модула. Генерираният API клиент, основната Java библиотека, която съдържа ръчно написани помощни функции за по‑лесна работа с API‑то, и модулът `pubsub`, който е библиотека за абониране за потоци от промени.

- [Документация за API клиент библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация за основната библиотека, включително примери с SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация за PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клиента има три класа – `DefaultApi`, `PublicApi` и `ModerationApi`. Класът `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикват директно от браузър/мобилно устройство и т.н. без автентикация.

`ModerationApi` предоставя обширен набор от живи и бързи API‑та за модерация. Всеки метод от `ModerationApi` приема параметър `sso` и може да се автентифицира чрез SSO или с cookie от сесия в FastComments.com.