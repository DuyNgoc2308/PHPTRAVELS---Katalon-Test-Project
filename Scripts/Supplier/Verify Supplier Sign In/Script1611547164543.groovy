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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

WebUI.openBrowser(null)

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://www.phptravels.net/supplier')

WebUI.setText(findTestObject('Object Repository/Page_Supplier Login/Page_Supplier Login/input_Login Panel_email'), email)

WebUI.setText(findTestObject('Object Repository/Page_Supplier Login/Page_Supplier Login/input_Login Panel_password'), password)

WebUI.click(findTestObject('Object Repository/Page_Supplier Login/Page_Supplier Login/button_Login'))

if (WebUI.verifyElementPresent('Object Repository/Page_Supplier Login/alert', 30)) {
    KeywordUtil.markFailed("FAILED: Login failed")
	WebUI.closeBrowser()
} 
else {
    WebUI.click(findTestObject('Object Repository/Page_Dashboard/strong_Logout'))
}

WebUI.closeBrowser()