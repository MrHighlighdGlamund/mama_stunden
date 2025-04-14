package dev.dioxus.main;

// need to re-export buildconfig down from the parent
import com.sohnidas_studios.boonk_arbeitszeit.BuildConfig;
import android.os.Bundle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import android.provider.Settings
import android.widget.Toast
import android.content.Intent
import android.app.Activity
import androidx.appcompat.app.AppCompatActivity
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import android.os.Build
import android.os.Environment
import android.net.Uri
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import android.Manifest

typealias BuildConfig = BuildConfig;


class MainActivity : WryActivity() {
    private lateinit var rustBridge: RustBridge
   // private lateinit var permissionLauncher: ActivityResultLauncher<Intent>

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Initialize the RustBridge to call Rust code
        rustBridge = RustBridge()
        
        GlobalScope.launch(Dispatchers.IO) {
            rustBridge.callRustFunction("Hello from Kotlin!")
        }
        // permissionLauncher = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
        //     if (hasPermission()) {
        //         Toast.makeText(this, "Permission granted", Toast.LENGTH_SHORT).show()
        //     } else {
        //         Toast.makeText(this, "Permission denied", Toast.LENGTH_SHORT).show()
        //     }
        // }

        // if (!hasPermission()) {
        //     requestFilePermission()
        // } else {
        //     Toast.makeText(this, "Permission already granted", Toast.LENGTH_SHORT).show()
        // }    
      }
    // private fun hasPermission(): Boolean {
    //     return ContextCompat.checkSelfPermission(
    //         this,
    //         Manifest.permission.MANAGE_EXTERNAL_STORAGE
    //     ) == PackageManager.PERMISSION_GRANTED
    // }

    // private fun requestFilePermission() {
    //     val uri = Uri.parse("package:${BuildConfig.APPLICATION_ID}")
    //     val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION, uri)

    //     permissionLauncher.launch(intent)
    // }
  }

