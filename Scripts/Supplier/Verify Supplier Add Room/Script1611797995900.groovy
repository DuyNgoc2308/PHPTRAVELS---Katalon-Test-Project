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

//WebUI.openBrowser('')
//
//WebUI.maximizeWindow()
//
//WebUI.navigateToUrl('https://www.phptravels.net/supplier')

WebUI.callTestCase(findTestCase('Supplier/Verify Supplier Sign In'), [('email') : email, ('password') : password], FailureHandling.STOP_ON_FAILURE)


WebUI.click(findTestObject('Object Repository/Page_Dashboard/Page_Dashboard/a_Hotels'))

WebUI.click(findTestObject('Object Repository/Page_Dashboard/Page_Dashboard/a_Add Room'))

WebUI.click(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/span_'))

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Price_basicprice'), price)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Quantity_quantity'), quantity)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Min Stay_minstay'), minStay)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Max Adults_adults'), adults)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Max Child_children'), children)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_No. of Exrta Beds_extrabeds'), extraBed)

WebUI.setText(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/input_Extra Bed Charges_bedcharges'), bedCharge)

WebUI.click(findTestObject('Object Repository/Page_Dashboard/Page_Add Room/button_Submit'))

WebUI.closeBrowser()

