Προσθέστε το FastComments SDK στο αρχείο `build.gradle.kts` της εφαρμογής σας:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Βεβαιωθείτε ότι έχετε το Repsy repository στο `settings.gradle.kts` του project σας:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```