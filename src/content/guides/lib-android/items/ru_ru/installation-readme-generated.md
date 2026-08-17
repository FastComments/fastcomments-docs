Добавьте FastComments SDK в файл `build.gradle.kts` вашего приложения:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Убедитесь, что в `settings.gradle.kts` вашего проекта указан репозиторий Repsy:

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