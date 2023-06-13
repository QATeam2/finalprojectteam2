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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demo-app.coding.id/')

WebUI.click(findTestObject('Object Repository/Login/Page_Be a Profressional Talent with Coding.ID/btn_Masuk'))

WebUI.setText(findTestObject('Object Repository/Login/Page_Masuk untuk dapatkan akses di Coding.ID/txt_email'), 'ragil.irvandi97@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Login/Page_Masuk untuk dapatkan akses di Coding.ID/txt_password'), 
    'QgfuYk5Tsdi8mqoM6vPKkQ==')

WebUI.click(findTestObject('Object Repository/Login/Page_Masuk untuk dapatkan akses di Coding.ID/btn_Login'))

WebUI.click(findTestObject('Login/Verify Login/Page_Be a Profressional Talent with Coding.ID/i_user'))

WebUI.verifyElementPresent(findTestObject('Login/Verify Login/Page_Be a Profressional Talent with Coding.ID/a_Logout'), 
    1, FailureHandling.STOP_ON_FAILURE)

