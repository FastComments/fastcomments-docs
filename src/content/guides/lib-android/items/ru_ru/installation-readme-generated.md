Добавьте FastComments SDK в файл `build.gradle.kts` вашего приложения:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Убедитесь, что репозиторий Repsy присутствует в файле `settings.gradle.kts` вашего проекта:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // другие репозитории...
    }
}
```