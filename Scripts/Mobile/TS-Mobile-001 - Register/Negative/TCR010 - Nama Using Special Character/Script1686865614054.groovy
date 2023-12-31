import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Mobile.startApplication('Data Files\\DemoAppV2.apk', true)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn-Login Here'), 0)

Mobile.tap(findTestObject('Mobile/Register/btn- Register, now'), 0)

Mobile.setText(findTestObject('Mobile/Register/txt-Nama'), '!@#$%', 0)

Mobile.takeScreenshot('', FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn- Birth Date'), 0)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn.Year'), 0)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn.Month'), 0)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn.Date'), 0)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/btn- OK'), 0)

Mobile.setText(findTestObject('Mobile/Register/txt-Email'), 'ragil.irvandi97@gmail.com', 0)

Mobile.setText(findTestObject('Mobile/Register/txt-WhatsApp'), '123456789', 0)

Mobile.setText(findTestObject('Mobile/Register/txt-Password'), 'madesu123', 0)

Mobile.setText(findTestObject('Mobile/Register/txt-Confirm Password'), 'madesu123', 0)

Mobile.tap(findTestObject('Object Repository/Mobile/Register/CheckBox'), 0)

Mobile.tap(findTestObject('Mobile/Register/btn-Daftar'), 0)

Mobile.verifyElementText(findTestObject('Mobile/Register/Verification Name must be alphabetic'), 'Name must be alphabetic')

Mobile.closeApplication()

