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

WebUI.navigateToUrl('http://23.98.68.185/Login/LogOut')

WebUI.setText(findTestObject('Object Repository/Search Client Hoxton/Page_HOXTON/input_Login_UserName'), 'rukhsana.parveen@xceedance.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Search Client Hoxton/Page_HOXTON/input_Login_password'), 'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_HOXTON/span_Client Management'))

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_HOXTON/a_Search Client'))

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_Search Client  HOXTON/a_Search'))

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_Search Client  HOXTON/button_CL0004687'))

WebUI.setText(findTestObject('Object Repository/Search Client Hoxton/Page_Edit Client  HOXTON/input__DOB'), '10/10/1968')

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_Edit Client  HOXTON/a_10'))

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_Edit Client  HOXTON/span_Save'))

WebUI.click(findTestObject('Object Repository/Search Client Hoxton/Page_Edit Client  HOXTON/button_YES'))

WebUI.closeBrowser()

