package megadrile.stats

final class DosageStore(val nSamples: Int) {
  private val bytes: Array[Byte] = new Array((nSamples + 3)/4)

  def setDosage(i: Int, dosage: Byte): Unit = {
    val iByte = i / 4
    val byte = bytes(iByte)
    val byteNew = i % 4 match {
      case 0 => ((byte & 252) + dosage).toByte
      case 1 => ((byte & 243) + (dosage << 2)).toByte
      case 2 => ((byte & 207) + (dosage << 4)).toByte
      case 3 => ((byte & 63) + (dosage << 6)).toByte
    }
    bytes(iByte) = byteNew
  }
  def getDosage(i: Int): Byte = {
    val iByte = i / 4
    val byte = bytes(iByte)
    i % 4 match {
      case 0 => (byte & 3).toByte
      case 1 => ((byte & 12) >> 2).toByte
      case 2 => ((byte & 48) >> 4).toByte
      case 3 => ((byte & 192) >> 6).toByte
    }
  }
}

object DosageStore {
  def apply(nSamples: Int): DosageStore = new DosageStore(nSamples)
}