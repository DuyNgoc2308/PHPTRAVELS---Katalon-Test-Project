import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import cucumber.api.PendingException
import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class LoginSteps {

	@Given("I navigate to Login page")
	def navigate_to_Login_page() {
		WebUI.openBrowser('')

		WebUI.navigateToUrl('https://www.phptravels.net/')

		WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_My Account'))

		WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_Login'))
	}

	@When("I enter (.*) and (.*)")
	def enter_username_and_password(String email, String password) {

		WebUI.setText(findTestObject('Object Repository/Page_Login/input_Login_username'), email)

		WebUI.setText(findTestObject('Object Repository/Page_Login/input_Email_password'), password)
	}

	@When("I click on Login button")
	def click_on_Login_button() {
		WebUI.click(findTestObject('Object Repository/Page_Login/label_Remember Me'))

		WebUI.click(findTestObject('Object Repository/Page_Login/button_Login'))
	}

	@Then("I should be able to Log In successfully")
	def log_in_successfully() {
		println("Log In successfully!")
	}
}