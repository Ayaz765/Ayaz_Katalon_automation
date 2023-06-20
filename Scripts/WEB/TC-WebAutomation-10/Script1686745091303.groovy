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

WebUI.navigateToUrl('https://www.globalsqa.com/angularJs-protractor/registration-login-example/#/login')

WebUI.setText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Username_username'), 
    'ayaz')

WebUI.setEncryptedText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Password_password'), 
    'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/a_Register'))

WebUI.setText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_First name_firstName'), 
    'ayaza')

WebUI.setText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Last name_lastName'), 
    'ahmad')

WebUI.setText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Username_username'), 
    'ayaz123')

WebUI.setEncryptedText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Password_password'), 
    'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/button_Register'))

WebUI.setText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Username_username'), 
    'ayaz123')

WebUI.click(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/form_Username                        Userna_9e8e15'))

WebUI.setEncryptedText(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/input_Password_password'), 
    'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Registration/Page_AngularJS User Registration and Login Example/a_Logout'))

WebUI.closeBrowser()

