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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.phptravels.net/')

WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_My Account'))

WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_Login'))

WebUI.setText(findTestObject('Object Repository/Page_Login/input_Login_username'), email)

WebUI.setText(findTestObject('Object Repository/Page_Login/input_Email_password'), password)

WebUI.click(findTestObject('Object Repository/Page_Login/button_Login'))

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Login/alert'), 30)) {
    WebUI.closeBrowser()

    KeywordUtil.markFailed('ERROR: This account does not exit')
} else {
    WebUI.click(findTestObject('Object Repository/Page_Register/Page_My Account/a_John'))

    WebUI.click(findTestObject('Object Repository/Page_Register/Page_My Account/a_Logout'))
}

WebUI.closeBrowser()