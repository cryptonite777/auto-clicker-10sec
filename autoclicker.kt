import java.awt.Robot
import java.awt.event.InputEvent
import java.io.BufferedReader
import java.io.InputStreamReader

fun minecraftIsActive(): Boolean {
    val process = ProcessBuilder(
        "xdotool",
        "getactivewindow",
        "getwindowname"
    ).start()

    val title = BufferedReader(
        InputStreamReader(process.inputStream)
    ).readLine() ?: ""

    process.waitFor()

    return title.contains("Minecraft", ignoreCase = true)
}

fun main() {
    val robot = Robot()

    println("Auto-clicker running.")
    println("Only clicks when Minecraft is active.")
    println("Ctrl+C to stop.")

    while (true) {
        if (minecraftIsActive()) {
            robot.mousePress(InputEvent.BUTTON1_DOWN_MASK)
            robot.mouseRelease(InputEvent.BUTTON1_DOWN_MASK)

            println("Clicked!")
        }

        Thread.sleep(10_000)
    }
}
