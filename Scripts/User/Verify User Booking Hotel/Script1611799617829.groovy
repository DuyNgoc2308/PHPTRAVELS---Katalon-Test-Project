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




WebUI.openBrowser(null)

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://www.phptravels.net/')

WebUI.click(findTestObject('Object Repository/Page_Home_Searching/input_search'))

WebUI.click(findTestObject('Object Repository/Page_Home_Searching/div_match search result'))

WebUI.setText(findTestObject('Object Repository/Page_Home_Searching/input_checkin'), '01/02/2021')

WebUI.setText(findTestObject('Object Repository/Page_Home_Searching/input_checkout'), '04/05/2021')

WebUI.click(findTestObject('Object Repository/Page_Home_Searching/button_adults_plus'))

WebUI.click(findTestObject('Object Repository/Page_Home_Searching/button_children_plus'))

WebUI.click(findTestObject('Object Repository/Page_Home_Searching/button_search'))

//
//WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/Page_Tria Hotel Istanbul Special/label_Select Room'))
//
//WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/Page_Tria Hotel Istanbul Special/button_Book Now'))
//
//WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/Page_Tria Hotel Istanbul Special/button_CONFIRM THIS BOOKING'))
//
//WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/Page_Invoice/button_Pay on Arrival'))

WebUI.closeBrowser()



