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

Потім додайте залежності, які вам потрібні:

```xml
<dependencies>
    <!-- Клієнт API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Основна бібліотека (включає SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Бібліотека PubSub (для подій у реальному часі) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // Основна бібліотека (включає SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Бібліотека PubSub (для подій у реальному часі)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Зміст бібліотеки

Ця бібліотека містить три модулі. Згенерований клієнт API, базова Java-бібліотека, яка містить вручну написані утиліти для спрощення роботи з API, та модуль `pubsub`, який є бібліотекою для підписки на потоки змін.

- [Документація бібліотеки клієнта API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документація основної бібліотеки, включно з прикладами SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документація бібліотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публічні та захищені API

Для клієнта API є два класи, `DefaultApi` та `PublicApi`. `DefaultApi` містить методи, які вимагають вашого API-ключа, а `PublicApi` містить виклики API, які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.