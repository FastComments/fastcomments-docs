Добавьте FastComments SDK в файл `build.gradle.kts` вашего приложения:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Убедитесь, что репозиторий Repsy указан в файле `settings.gradle.kts` вашего проекта:

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