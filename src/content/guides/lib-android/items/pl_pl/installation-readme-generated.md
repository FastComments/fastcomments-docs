Dodaj FastComments SDK do pliku `build.gradle.kts` swojej aplikacji:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Upewnij się, że masz repozytorium Repsy w pliku `settings.gradle.kts` swojego projektu:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // inne repozytoria...
    }
}
```