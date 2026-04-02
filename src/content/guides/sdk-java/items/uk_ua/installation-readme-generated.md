### Maven

Додайте репозиторій Repsy до POM вашого проєкту:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Потім додайте необхідні залежності:

```xml
<dependencies>
    <!-- Клієнт API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Основна бібліотека (включає SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Бібліотека PubSub (для живих подій) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додайте репозиторій Repsy до файлу build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Клієнт API
    implementation "com.fastcomments:client:1.3.2"
    
    // Основна бібліотека (включає SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Бібліотека PubSub (для живих подій)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

Ця бібліотека містить три модулі. Згенерований клієнт API, основна Java-бібліотека, яка містить вручну написані утиліти, що полегшують роботу з API, та модуль `pubsub`, який є бібліотекою для підписки на потоки змін.

- [Документація бібліотеки клієнта API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документація основної бібліотеки, включаючи приклади SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документація бібліотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публічні та захищені API

Для клієнта API існують два класи: `DefaultApi` та `PublicApi`. `DefaultApi` містить методи, які вимагають вашого API-ключа, а `PublicApi` містить виклики API, які можна виконувати без аутентифікації безпосередньо з браузера, мобільного пристрою тощо.