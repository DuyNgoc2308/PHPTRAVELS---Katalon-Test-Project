package user
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

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class SignupSteps {
	@Given("I navigate to Sign Up page")
	def user_navigates_to_Sign_up_page(){
		WebUI.openBrowser('')

		WebUI.navigateToUrl('https://www.phptravels.net/')

		WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_My Account'))

		WebUI.click(findTestObject('Object Repository/Page_PHPTRAVELS  Travel Technology Partner/a_Sign Up'))
	}

	@When("I enter (.*), (.*), (.*)")
	def user_enters_firstname_lastname_phone(String firstname, String lastname, String phone){


		WebUI.setText(findTestObject('Object Repository/Page_Register/input_Sign Up_firstname'), firstname)


		WebUI.setText(findTestObject('Object Repository/Page_Register/input_First Name_lastname'), lastname)


		WebUI.setText(findTestObject('Object Repository/Page_Register/input_Last Name_phone'), phone)
	}
	@When("I type (.*), (.*)")
	def types_email_password(String email, String password){

		WebUI.setText(findTestObject('Object Repository/Page_Register/input_Mobile Number_email'), email)
		WebUI.setText(findTestObject('Object Repository/Page_Register/input_Email_password'), password)

		WebUI.setText(findTestObject('Object Repository/Page_Register/input_Password_confirmpassword'), password)
	}
	@When("I click on Sign Up button")
	def clicks_on_Sign_Up_button(){
		WebUI.click(findTestObject('Object Repository/Page_Register/button_Sign Up'))
	}
	@Then("I should be able to Sign Up sucessfully")
	def sign_up_sucessfully(){
		WebUI.closeBrowser()
		println("Sign up successfully!")
	}
}