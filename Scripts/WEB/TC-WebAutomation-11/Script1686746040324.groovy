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

WebUI.navigateToUrl('https://www.globalsqa.com/angularJs-protractor/Multiform/#/form/profile')

WebUI.setText(findTestObject('Object Repository/Friend/Page_/input_Name_name'), 'ayaz ahmad')

WebUI.setText(findTestObject('Object Repository/Friend/Page_/input_Email_email'), 'ayazahmad984@gmail.com')

WebUI.click(findTestObject('Object Repository/Friend/Page_/span_Next Section_glyphicon glyphicon-circl_68c672'))

WebUI.click(findTestObject('Object Repository/Friend/Page_/div_I like XBOX'))

WebUI.click(findTestObject('Object Repository/Friend/Page_/input_I like XBOX_009'))

WebUI.click(findTestObject('Object Repository/Friend/Page_/a_Next Section'))

WebUI.click(findTestObject('Object Repository/Friend/Page_/button_Submit'))

WebUI.closeBrowser()

