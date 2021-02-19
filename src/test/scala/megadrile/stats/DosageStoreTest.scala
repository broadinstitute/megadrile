package megadrile.stats

import org.scalatest.funsuite.AnyFunSuite

final class DosageStoreTest extends AnyFunSuite{
  private def assertStoreAndRetrieve(store: DosageStore, i: Int, dosage: Byte): Unit = {
    store.setDosage(i, dosage)
    val dosageRetrieved = store.getDosage(i)
    assert(dosageRetrieved == dosage, s"For i=$i and dosage=$dosage")
  }
  test("Store and retrieve") {
    val nSamples: Int = 19
    val store = DosageStore(nSamples)
    for(i <- 1 to 100) {
      assertStoreAndRetrieve(store, i % 19, (i % 3).toByte)
    }
  }

}
