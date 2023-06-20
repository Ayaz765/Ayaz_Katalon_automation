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

println('This is Url' + GlobalVariable.Url) 
WebUI.navigateToUrl(GlobalVariable.Url)

WebUI.setText(findTestObject('Object Repository/Page_HOXTON/input_Login_UserName'), 'ayazahmad984@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_HOXTON/input_Login_password'), 'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Page_HOXTON/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_HOXTON/a_Client Management'))

WebUI.click(findTestObject('Object Repository/Page_HOXTON/a_Add New Client'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Add Client  HOXTON/select_Select                              _534643'), 
    'Mr', true)

WebUI.setText(findTestObject('Object Repository/Page_Add Client  HOXTON/input__firstName'), 'Demo')

WebUI.setText(findTestObject('Object Repository/Page_Add Client  HOXTON/input__lastName'), 'demo one')

WebUI.setText(findTestObject('Object Repository/Page_Add Client  HOXTON/input__DOB'), '09/09/1969')

WebUI.click(findTestObject('Object Repository/Page_Add Client  HOXTON/a_9'))

WebUI.setText(findTestObject('Object Repository/Page_Add Client  HOXTON/input__autocomplete'), '8 lin')

WebUI.click(findTestObject('Object Repository/Page_Add Client  HOXTON/li_8 Hei - Lin Way, Ludgershall, Andover SP11 9QH'))

WebUI.click(findTestObject('Object Repository/Page_Add Client  HOXTON/button_Submit                    Save'))

WebUI.click(findTestObject('Object Repository/Page_Add Client  HOXTON/button_YES'))

WebUI.closeBrowser()

